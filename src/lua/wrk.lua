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

function format()
  if wrk.headers ~= nil then
    if not wrk.headers["Host"] then
      wrk.headers["Host"] = wrk.host
    end
  end

  return wrk
end

function request()
  return format()
end

function init(args)
end
