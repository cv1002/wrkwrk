wrk = {
  scheme  = "http",
  host    = "localhost",
  port    = 80,
  method  = "GET",
  path    = "",
  headers = {},
  body    = nil,
  timeout = 30000,
  version = "HTTP1.1",
}

function format(scheme, host, port, method, url, headers, body, timeout, version)
  local host    = host or wrk.host
  local scheme  = scheme or wrk.scheme
  local port    = port or wrk.port
  local method  = method or wrk.method
  local url     = url or wrk.path
  local headers = headers or wrk.headers
  local body    = body or wrk.body
  local timeout = timeout or wrk.timeout
  local version = version or wrk.version

  if headers ~= nil then
    if not headers["Host"] then
      wrk.headers["Host"] = host
    end
  end

  return {
    scheme  = scheme,
    host    = host,
    port    = port,
    method  = method,
    url     = url,
    headers = headers,
    body    = body,
    timeout = timeout,
    version = version,
  }
end

function request()
  return format()
end

function init(args)
end
