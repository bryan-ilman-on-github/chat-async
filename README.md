# Experiments

## 2.1. Original code of broadcast chat.

### server

![alt text](./pngs/2.1a.png)

### clients

![alt text](./pngs/2.1b.png)

Dari output, kita dapat mengamati bahwa server bertindak sebagai listener. Dengan demikian, server bisa menerima koneksi dari setiap klien. Ketika server menerima pesan baru dari sebuah klien, server akan meneruskannya kepada klien lainnya, yang disebut sebagai message forwarding. Dengan kata lain, server melakukan broadcast atas pesan yang diterimanya. Ini menunjukkan bahwa server tidak hanya menerima, tetapi juga menyebarkan informasi di antara klien-klien yang terhubung.
