# Readme

* docker build --tag first_log:0.1.0 .
* docker tag first_log:0.1.0 192.168.1.102:5000/first_log:0.1.0
* docker push 192.168.1.102:5000/first_log:0.1.0
* `curl --cacert ~/tmp/k8s_ca.crt  https://192.168.1.102:5000/v2/_catalog`
* kubectl create deployment first-log --image=192.168.1.102:5000/first_log:0.1.0 --replicas=2
