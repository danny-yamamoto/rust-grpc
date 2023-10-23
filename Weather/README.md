## install protoc
```bash
sudo apt-get update
sudo apt-get install -y protobuf-compiler
```

## install grpcurl
```bash
# install asdf
git clone https://github.com/asdf-vm/asdf.git ~/.asdf --branch v0.13.1
asdf
. "$HOME/.asdf/asdf.sh"
asdf
# install grpcrl
asdf plugin add grpcurl
asdf list all grpcurl
asdf install grpcurl 1.8.8
asdf local grpcurl 1.8.8
# check api
grpcurl -plaintext localhost:50051 list
grpcurl -plaintext -d '{"name": "World"}' localhost:50051 hello.HelloService/SayHello
```

## 
```bash
grpcurl -plaintext -d '{"condition": "Cloudy"}' localhost:8080 weather.v1.WeatherService/Weather
{
  "text": "Today's weather is Cloudy!"
}
```
