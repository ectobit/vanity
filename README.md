# vanity

Go vanity imports HTTP server

## Environment variables

RUST_LOG=info

## Related blog posts

- [Vanity Go Import Paths](https://blog.bramp.net/post/2017/10/02/vanity-go-import-paths/)
- [Vanity import paths in Go](https://sagikazarmark.hu/blog/vanity-import-paths-in-go/)

## Related projects

- [Go Vanity URLs](https://github.com/GoogleCloudPlatform/govanityurls)
- [kkn.fi/vanity](https://github.com/kare/vanity)
- [vanity](https://github.com/hawx/vanity)

## Install

```sh
helm plugin install https://github.com/aslafy-z/helm-git --version 0.10.0
helm repo add vanity git+https://github.com/acim/vanity@deploy/charts
helm install vanity/vanity
```

## Example responses

```txt
content-type: text/html; charset=utf-8
cache-control: public, max-age=0, must-revalidate
strict-transport-security: max-age=31536000

<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<meta name="go-import" content="emperror.dev/errors git https://github.com/emperror/errors">
<meta name="go-source" content="emperror.dev/errors https://github.com/emperror/errors https://github.com/emperror/errors/tree/master{/dir} https://github.com/emperror/errors/blob/master{/dir}/{file}#L{line}">
<style>
* { font-family: sans-serif; }
body { margin-top: 0; }
.content { display: inline-block; }
code { display: block; font-family: monospace; font-size: 1em; background-color: #d5d5d5; padding: 1em; margin-bottom: 16px; }
ul { margin-top: 16px; margin-bottom: 16px; }
</style>
</head>
<body>
<div class="content">
<h2>emperror.dev/errors</h2>
<code>go get emperror.dev/errors</code>
<code>import "emperror.dev/errors"</code>
Home: <a href="https://godoc.org/emperror.dev/errors">https://godoc.org/emperror.dev/errors</a><br/>
Source: <a href="https://github.com/emperror/errors">https://github.com/emperror/errors</a><br/>
</div>
</body>
</html>
```

```txt
<!DOCTYPE html>
<html>
    <head>
        <meta name="go-import" content="go.uber.org/zap git https://github.com/uber-go/zap">
        <meta name="go-source" content="go.uber.org/zap https://github.com/uber-go/zap https://github.com/uber-go/zap/tree/master{/dir} https://github.com/uber-go/zap/tree/master{/dir}/{file}#L{line}">
        <meta http-equiv="refresh" content="0; url=https://pkg.go.dev/go.uber.org/zap/errors">
    </head>
    <body>
        Nothing to see here. Please <a href="https://pkg.go.dev/go.uber.org/zap/errors">move along</a>.
    </body>
</html>
```
