import http.server
import json
import signal
import socketserver
import re
import sys
from urllib.parse import urlparse, parse_qs

nb=1

class CustomHandler(http.server.BaseHTTPRequestHandler):
	def do_HEAD(self):
		self.send_response(200)
		self.send_header('Content-type', 'application/json')
		self.end_headers()

	def do_GET(self):
		if self.path != "/":
			self.api_404()
		else:
			self.api_ok()

	def do_DELETE(self):
		global nb
		nb=0
		self.api_ok()

	def do_POST(self):
		global nb
		if self.path == "/set-by-query" or self.path == "/set-by-header" or self.path == "/set-by-body":
			self.send_response(405)
			self.send_header('Content-type', 'application/json')
			self.end_headers()
			self.wfile.write(b"\"405 Method not allowed\"\r\n")
			return
		nb+=1
		self.api_ok()

	def do_PUT(self):
		global nb
		url = urlparse(self.path)
		path = url.path
		if path.startswith("/set-by-path"):
			match = re.match("^/set-by-path/-?\d+$", self.path)
			if not match:
				self.send_response(400)
				self.send_header('Content-type', 'application/json')
				self.end_headers()
				self.wfile.write(b"\"400 Need a path as /set-by-path/:int\"\r\n")
				return
			nb = int(self.path[13:])
			self.api_ok()
		elif path == "/set-by-query":
			try:
				nb = int(parse_qs(url.query)["nb"][0])
				self.api_ok()
			except:
				self.send_response(400)
				self.send_header('Content-type', 'application/json')
				self.end_headers()
				self.wfile.write(b"\"400 Need integer in query with name nb\"\r\n")
		elif self.path == "/set-by-header":
			try:
				nb = int(self.headers["x-nb"])
				self.api_ok()
			except:
				self.send_response(400)
				self.send_header('Content-type', 'application/json')
				self.end_headers()
				self.wfile.write(b"\"400 Need integer in header with name x-nb\"\r\n")
		elif self.path == "/set-by-body":
			try:
				content_len = int(self.headers.get('Content-Length'))
				data = self.rfile.read(content_len)
				nb = int(data.decode())
				self.api_ok()
			except:
				self.send_response(400)
				self.send_header('Content-type', 'application/json')
				self.end_headers()
				self.wfile.write(b"\"400 Need integer in the body\"\r\n")
		else:
			self.api_404()

	def api_404(self):
		self.send_response(404)
		self.send_header('Content-type', 'application/json')
		self.end_headers()
		self.wfile.write(b"\"404 Path not found\"\r\n")

	def api_ok(self):
		global nb
		self.send_response(200)
		self.send_header('Content-type', 'application/json')
		self.end_headers()
		self.wfile.write(nb.__str__().encode())

with socketserver.TCPServer(("", 8000), CustomHandler) as httpd:
	httpd.timeout = 0.02
	print("Serving ...")
	def stop_server(signal, frame):
		httpd.server_close()
		print("Server stopped")
		sys.exit(0)

	signal.signal(signal.SIGINT, stop_server)
	httpd.serve_forever()
