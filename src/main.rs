use async_nats;
use warp::Filter;
use flatbuffers;

#[allow(dead_code, unused_imports)]
#[path = "./request_generated.rs"]
mod request_generated;
pub use request_generated::eisenbeton::wire::request::{root_as_eisen_request, EisenRequest, EisenRequestArgs};

#[allow(dead_code, unused_imports)]
#[path = "./response_generated.rs"]
mod response_generated;
pub use response_generated::eisenbeton::wire::response::{root_as_eisen_response};









#[tokio::main]
async fn main() {
    let opts = async_nats::Options::new();
    let nc = match opts.connect("localhost").await {
        Ok(nc) => nc,
        Err(_) => panic!("Could not establish nats connection"),
    };
    let handler = warp::path!(String)
        .and(with_nats(nc))
        .and(warp::method())
        .and(warp::body::bytes())
        .and(warp::header::optional::<String>("content-type"))
        .and_then(nats_handler);
    warp::serve(handler).run(([0,0,0,0], 8500)).await;
}

fn with_nats(nc: async_nats::Connection) -> impl Filter<Extract = (async_nats::Connection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || nc.clone())
}

async fn nats_handler(
    topic: String,
    nc: async_nats::Connection, 
    method: warp::hyper::Method, 
    body: warp::hyper::body::Bytes, 
    content_type: Option<String>) -> std::result::Result<impl warp::Reply, warp::Rejection> {
        let t = format!("/{}", topic);
        let msg = build_eisen_request(&t, &t, &method.to_string(), &content_type.unwrap_or(String::default()), body);
        let res = nc.request(&t, msg).await;
        return match res {
            Ok(msg) => {
                let m = open_eisen_reponse(&msg);
                Ok(m.to_vec())
            },
            Err(_) =>  Ok(b"".to_vec())
        }
    }

fn build_eisen_request<'a>(
    path:&'a String, 
    uri:&'a String, 
    method:&'a String, 
    content_type:&'a String,
    content: warp::hyper::body::Bytes) -> Vec<u8> {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);
    let path_ = builder.create_string(path);
    let url_ = builder.create_string(uri);
    let method_ = builder.create_string(method);
    let content_type_ = builder.create_string(content_type);
    let content_ = builder.create_vector(&content[..]);
    let req = EisenRequest::create(&mut builder, &EisenRequestArgs{
        uri: Some(url_),
        path: Some(path_),
        method: Some(method_),
        content_type: Some(content_type_),
        content: Some(content_),
        ..Default::default()
    });
    builder.finish(req, None);
    return builder.finished_data().to_vec();

}


fn open_eisen_reponse(msg:&async_nats::Message) -> &[u8] {
    let empty = b"";
    match root_as_eisen_response(&msg.data) {
        Ok(resp) => 
            match resp.content() {
                Some(bytes) => bytes,
                None => empty
            },
        Err(_) => empty
    }
}



