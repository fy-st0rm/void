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

const handle_form = async () => {
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

const add_event_listener = (element) => {
	element.addEventListener("click", async () => {
		await fetch(`/api_download_file/${element.id}`, { method: "GET" })
			.then(res => res.blob())
			.then(blob => {
				let url = window.URL.createObjectURL(blob);

				const link = document.createElement('a');
				link.href = url;
				link.setAttribute('download', element.name);
				document.body.appendChild(link);
				link.click();
			})
			.catch(err => alert(err));
	});
}

const display_files = (files) => {
	const div = document.getElementById("files");

	for (let id in files) {
		let name = files[id];

		let anchor = document.createElement("a");
		anchor.setAttribute("id", id);
		anchor.setAttribute("name", name);
		anchor.setAttribute("href", "#");
		anchor.innerHTML = name;

		let br = document.createElement("br");
		div.appendChild(anchor);
		div.appendChild(br);

		add_event_listener(anchor);
	}
}

window.onload = async () => {
	const user_details_p = document.getElementById("user_details");
	let id = localStorage.getItem("id");
	let name = localStorage.getItem("name");
	user_details_p.innerHTML = `
		Name: ${name}<br>
		Id: ${id}<br>
	`;

	let response = await fetch(`/api_dashboard/${id}`, { method: "GET" });
	let text = await response.text();
	let data = JSON.parse(text);
	if (!response.ok) {
		alert(data.msg);
		return;
	}

	display_files(data);

	await handle_form();
}
