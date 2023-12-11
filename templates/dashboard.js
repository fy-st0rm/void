function handleSubmit(event) {
	const form = event.currentTarget;
	const formData = new FormData(form);
	const searchParams = new URLSearchParams(formData);

	const fetchOptions = {
		method: "POST",
	};

	if (form.method.toLowerCase() === 'post') {
		if (form.enctype === 'multipart/form-data') {
			fetchOptions.body = formData;
		} else {
			fetchOptions.body = searchParams;
		}
	} else {
		url.search = searchParams;
	}

	fetch(url, fetchOptions);

	event.preventDefault();
}

const file_register = async (filename) => {
	const params = {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
			"Accept": "*/*",
		},
		body: JSON.stringify({
			owner_id: localStorage.getItem("id"),
			filename: filename
		})
	}

	let response = await fetch("/api_upload_register", params);
	return response;
}

const upload_file = async (form) => {
	const form_data = new FormData(form);

	const params = {
		method: "POST",
		body: form_data
	};

	let response = await fetch("/api_upload_file", params);
	return response;
}

window.onload = () => {
	const user_details_p = document.getElementById("user_details");
	let id = localStorage.getItem("id");
	let name = localStorage.getItem("name");
	user_details_p.innerHTML = `
		Name: ${name}<br>
		Id: ${id}<br>
	`;

	const form = document.getElementById("form");
	form.addEventListener("submit", async (e) => {
		e.preventDefault();
		filename = document.getElementById("file_upload").value;
		filename = filename.split("\\").pop()
	
		let response = await file_register(filename);
		if (!response.ok) {
			let result = await response.text();
			let data = JSON.parse(result);
			alert(data.msg);
			return;
		}
	
		response = await upload_file(form);
		let result = await response.text();
		alert(result);

		location.reload();
	})
}
