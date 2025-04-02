# Before Threads
## 500 requests, 50 concurrentes, i = 1.000.000
```text
Server Software:
Server Hostname:        localhost
Server Port:            8080

Document Path:          /pi/1000000
Document Length:        Variable

Concurrency Level:      50
Time taken for tests:   104.308 seconds
Complete requests:      500
Failed requests:        0
Total transferred:      44922 bytes
HTML transferred:       36422 bytes
Requests per second:    4.79 [#/sec] (mean)
Time per request:       10430.762 [ms] (mean)
Time per request:       208.615 [ms] (mean, across all concurrent requests)
Transfer rate:          0.42 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.4      0       2
Processing:   208 9896 1853.1  10392   10698
Waiting:      208 9896 1853.1  10392   10698
Total:        210 9896 1852.8  10392   10698

Percentage of the requests served within a certain time (ms)
  50%  10392
  66%  10488
  75%  10524
  80%  10546
  90%  10595
  95%  10633
  98%  10687
  99%  10690
 100%  10698 (longest request)
```
# After Threads
## 500 requests, 50 concurrentes, i = 1.000.000
```text
Server Software:
Server Hostname:        localhost
Server Port:            8080

Document Path:          /pi/1000000
Document Length:        Variable

Concurrency Level:      50
Time taken for tests:   22.796 seconds
Complete requests:      500
Failed requests:        0
Total transferred:      44438 bytes
HTML transferred:       35938 bytes
Requests per second:    21.93 [#/sec] (mean)
Time per request:       2279.634 [ms] (mean)
Time per request:       45.593 [ms] (mean, across all concurrent requests)
Transfer rate:          1.90 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    1   1.5      0       6
Processing:   240 2210 947.1   2327    3897
Waiting:      238 2208 945.9   2318    3897
Total:        240 2211 947.4   2327    3897

Percentage of the requests served within a certain time (ms)
  50%   2327
  66%   2789
  75%   2998
  80%   3130
  90%   3395
  95%   3538
  98%   3673
  99%   3756
 100%   3897 (longest request)
```
## 1000 requests, 100 concurrentes, i = 1.000.000
```text
Server Software:
Server Hostname:        localhost
Server Port:            8080

Document Path:          /pi/1000000
Document Length:        Variable

Concurrency Level:      100
Time taken for tests:   45.609 seconds
Complete requests:      1000
Failed requests:        0
Total transferred:      88576 bytes
HTML transferred:       71576 bytes
Requests per second:    21.93 [#/sec] (mean)
Time per request:       4560.883 [ms] (mean)
Time per request:       45.609 [ms] (mean, across all concurrent requests)
Transfer rate:          1.90 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    1   1.4      0       6
Processing:   244 4444 1458.3   4633    7590
Waiting:      239 4439 1456.8   4629    7590
Total:        244 4444 1458.4   4633    7590

Percentage of the requests served within a certain time (ms)
  50%   4633
  66%   5265
  75%   5557
  80%   5740
  90%   6153
  95%   6354
  98%   6809
  99%   7112
 100%   7590 (longest request)
```

## Conclusión
* Sin utilizar threads, una serie de requests tarda 10430.762ms, y en promedio cada request individual tarda 208.615ms.
* Al utilizar threads, dicha serie de requests tarda 2279.634, y en promedio cada request individual tarda 45.593ms.

Esto se debe a que, sin threads, las requests llegan y se "encolan". 
Por tanto, la segunda request va a acumular la duración de la anterior y su propio cálculo de pi.
La tercera lo mismo, pero acumulando el tiempo de la primera y la segunda, y así sucesivamente.

Entonces, las últimas request añaden cierta ponderación al promedio que se toma de las requests individuales.
En cambio, al utilizar threads se procesan requests en paralelo, por lo cual cada una toma una cantidad
de tiempo similar, de manera que el promedio es menor.

Por el lado del tiempo total, si en el primer caso el tiempo de las request se acumula pero en el segundo 
hay duraciones parecidas, es de esperar que la suma total haya sido mayor sin paralelización.