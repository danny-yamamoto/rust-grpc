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
```

## 
```bash
grpcurl -plaintext localhost:8080 list
grpcurl -plaintext -d '{"condition": "Cloudy"}' localhost:8080 weather.v1.WeatherService/Weather
{
  "text": "Today's weather is Cloudy!"
}
```
