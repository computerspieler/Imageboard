function addCatalogEntry(title, img_path, content) {
	var entry = document.createElement("div");
	entry.className = "catalog-entry";
	
	var title_elt = document.createElement("a");
	//TODO
	//title_elt.href = "";
	title_elt.textContent = title;
	entry.appendChild(title_elt);
	
	var img_elt = document.createElement("img");
	img_elt.className = "catalog-entry-img";
	img_elt.src = img_path;
	title_elt.appendChild(img_elt);

	var content_elt = document.createElement("div");
	content_elt.className = "catalog-entry-text";
	content_elt.textContent = content;
	entry.appendChild(content_elt);

	document.getElementById("catalog")
		.appendChild(entry);
}

async function retrieveJSON(endpoint) {
	var uri = window.location.href.replace(/\?.*/g, '') + `api/` + endpoint;
	var req = await fetch(uri);
	return req.json();
}

function getParameterByName(name, url = window.location.href) {
	name = name.replace(/[\[\]]/g, '\\$&');
	var regex = new RegExp('[?&]' + name + '(=([^&#]*)|&|#|$)');
	var results = regex.exec(url);
	
	if (!results)
		return null;
	if (!results[2])
		return '';

	return decodeURIComponent(results[2].replace(/\+/g, ' '))
		.replace(/^[^=]*=/g, '');
}

window.onload = () => {
	var query_id = getParameterByName("board");
	retrieveJSON(query_id).then((board_info) => {
		document
			.getElementById("boardName")
			.textContent = board_info.name;
		
		for(var thread of board_info.threads) {
			addCatalogEntry(thread.text, thread.img, thread.posts[0].text);
		}
	});
};
