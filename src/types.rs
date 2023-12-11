use serde::{
	Serialize,
	Deserialize
};

#[macro_export]
macro_rules! VResponse{
	($c: expr, $($x: expr), *) => {
		{
			let mut resp = HashMap::new();
			$(
				resp.insert($x.0, $x.1);
			)*
			HttpResponse::build($c).json(resp)
		}
	};
}


#[derive(Serialize, Deserialize)]
pub struct VLoginPayload {
	pub name: String,
	pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct VSignupPayload {
	pub name: String,
	pub password: String
}

