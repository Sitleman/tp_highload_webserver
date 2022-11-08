## Setup
#### Start rust webserver:
    docker build -t rustws -f Dockerfile .
    docker run -p 8081:8081 -v /home/nikita/rust/httptest:/httptest:ro --name rustws_container -t rustws

#### Start nginx webserver:
    docker build -t nginxws -f Dockerfile.nginx .
    docker run -p 8082:80 -v /home/nikita/rust/httptest:/var/www/html/httptest:ro --name nginxws_container -t nginxws

## Benchmark
#### rustws
    ab -n 10000 -c 20 127.0.0.1:8081/httptest/wikipedia_russia.html
    This is ApacheBench, Version 2.3 <$Revision: 1843412 $>
    Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
    Licensed to The Apache Software Foundation, http://www.apache.org/
    
    Server Software:        n.gureev
    Server Hostname:        127.0.0.1
    Server Port:            8081
    
    Document Path:          /httptest/wikipedia_russia.html
    Document Length:        954824 bytes
    
    Concurrency Level:      20
    Time taken for tests:   160.810 seconds
    Complete requests:      10000
    Failed requests:        0
    Total transferred:      9549859967 bytes
    HTML transferred:       9548240000 bytes
    Requests per second:    62.19 [#/sec] (mean)
    Time per request:       321.620 [ms] (mean)
    Time per request:       16.081 [ms] (mean, across all concurrent requests)
    Transfer rate:          57994.04 [Kbytes/sec] received
    
    Connection Times (ms)
    min  mean[+/-sd] median   max
    Connect:        0    0   0.1      0       3
    Processing:    14  321 168.8    322     869
    Waiting:        1   39  28.5     36     174
    Total:         14  321 168.8    322     869
    
    Percentage of the requests served within a certain time (ms)
    50%    322
    66%    399
    75%    447
    80%    475
    90%    550
    95%    606
    98%    665
    99%    696
    100%    869 (longest request) 

####nginx
    ab -n 10000 -c 20 127.0.0.1:8082/httptest/wikipedia_russia.html
    This is ApacheBench, Version 2.3 <$Revision: 1843412 $>
    Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
    Licensed to The Apache Software Foundation, http://www.apache.org/

    Server Software:        nginx/1.23.1
    Server Hostname:        127.0.0.1
    Server Port:            8082
    
    Document Path:          /httptest/wikipedia_russia.html
    Document Length:        954824 bytes
    
    Concurrency Level:      20
    Time taken for tests:   163.480 seconds
    Complete requests:      10000
    Failed requests:        0
    Total transferred:      9550620000 bytes
    HTML transferred:       9548240000 bytes
    Requests per second:    61.17 [#/sec] (mean)
    Time per request:       326.960 [ms] (mean)
    Time per request:       16.348 [ms] (mean, across all concurrent requests)
    Transfer rate:          57051.42 [Kbytes/sec] received
    
    Connection Times (ms)
    min  mean[+/-sd] median   max
    Connect:        0    0   0.0      0       1
    Processing:    12  327 174.8    329     844
    Waiting:        1   38  29.1     37     171
    Total:         12  327 174.9    329     844
    
    Percentage of the requests served within a certain time (ms)
    50%    329
    66%    413
    75%    460
    80%    488
    90%    559
    95%    618
    98%    673
    99%    711
    100%    844 (longest request)

