use log::{debug, error};
use reqwest::redirect::Policy;

pub fn same_origin_redirect_policy() -> Policy {
    //allow redirect to same origin
    Policy::custom(|attempt| {
        if let Some(prev) = attempt.previous().get(0) {
            debug!("Redirect to {:?}", attempt.url().origin());
            debug!("Original request Host = {:?}", prev.origin());
            if attempt.previous().len() > 5 {
                error!("Exceed redirect limit(5)");
                attempt.stop()
            } else if prev.origin() != attempt.url().origin() {
                error!("Redirect to non-same origin resource server");
                attempt.stop()
            } else {
                attempt.follow()
            }
        } else {
            attempt.stop()
        }
    })
}
