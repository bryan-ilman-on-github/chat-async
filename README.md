# Experiments

## 2.1. Original code of broadcast chat.

### server

![alt text](./pngs/2.1a.png)

### clients

![alt text](./pngs/2.1b.png)

Dari output, kita dapat mengamati bahwa server bertindak sebagai listener. Dengan demikian, server bisa menerima koneksi dari setiap klien. Ketika server menerima pesan baru dari sebuah klien, server akan meneruskannya kepada klien lainnya, yang disebut sebagai message forwarding. Dengan kata lain, server melakukan broadcast atas pesan yang diterimanya. Ini menunjukkan bahwa server tidak hanya menerima, tetapi juga menyebarkan informasi di antara klien-klien yang terhubung.

## 2.2. Modifying the websocket port

Apabila kita mengubah port server dan client menjadi sama, aplikasi dapat berjalan dengan baik. Hal ini karena keduanya akan saling terhubung pada port yang sama, sehingga tidak ada masalah dalam proses komunikasi. Namun, jika port yang digunakan oleh client dan server tidak sama, maka aplikasi akan mengalami kesulitan dalam berkomunikasi. Hal ini karena client akan mencoba terhubung ke server pada port yang telah ditentukan. Jika port tersebut tidak cocok dengan port yang digunakan oleh server, maka akan muncul error.
