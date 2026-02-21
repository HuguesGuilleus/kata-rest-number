const headers = new Headers({ "Content-Type": "application/json" });
const methodNotAllowed = new Response(`"405 Method not allowed"\r\n`, {
	status: 405,
	headers,
});
const setByPathPattern = new URLPattern({
	pathname: "/set-by-path/:nb(-?\\d+)",
});
const setByPathPatternSlash = new URLPattern({
	pathname: "/set-by-path/:nb(-?\\d+)/",
});

let nb = 1;

Deno.serve(async (request: Request) => {
	const url = new URL(request.url);

	if (url.pathname == "/") {
		if (request.method == "GET" || request.method == "HEAD") {
			// do nothing
		} else if (request.method == "DELETE") {
			nb = 0;
		} else if (request.method == "POST") {
			nb++;
		}
	} else if (setByPathPatternSlash.test(url)) {
		return new Response(
			`"400 Need a path as /set-by-path/:int"\r\n`,
			{ status: 400, headers },
		);
	} else if (setByPathPattern.test(url)) {
		if (request.method != "PUT") return methodNotAllowed.clone();
		const nbString = setByPathPattern.exec(url)?.pathname.groups.nb;
		if (!nbString) {
			return new Response(
				`"400 Need a path as /set-by-path/:int"\r\n`,
				{ status: 400, headers },
			);
		}
		nb = parseInt(nbString);
	} else if (url.pathname == "/set-by-query") {
		if (request.method != "PUT") return methodNotAllowed.clone();
		const nbString = url.searchParams.get("nb");
		if (!nbString || /^\d+$/.test(nbString)) {
			return new Response(
				`"400 Need integer in query with name nb"\r\n`,
				{ status: 400, headers },
			);
		}
		nb = parseInt(nbString);
	} else if (url.pathname == "/set-by-header") {
		if (request.method != "PUT") return methodNotAllowed.clone();
		const nbString = request.headers.get("x-nb");
		if (!nbString || /^\d+$/.test(nbString)) {
			return new Response(
				`"400 Need integer in header with name x-nb"\r\n`,
				{ status: 400, headers },
			);
		}
		nb = parseInt(nbString);
	} else if (url.pathname == "/set-by-body") {
		if (request.method != "PUT") return methodNotAllowed.clone();
		const nbString = await request.text();
		if (!nbString || /^\d+$/.test(nbString)) {
			return new Response(
				`"400 Need integer in the body"\r\n`,
				{ status: 400, headers },
			);
		}
		nb = parseInt(nbString);
	} else {
		return new Response(`"404 Path not found"\r\n`, {
			status: 404,
			headers,
		});
	}

	return new Response(nb + "", { status: 200, headers });
});
