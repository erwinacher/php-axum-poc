<?php

    // call the rust server endpoint
    $resp = file_get_contents("http://rust-server:3003/compute?name=World");
    $data = json_decode($resp, true);

?>
<!DOCTYPE html>
<html>
<head>
    <title>LAPP Hello World</title>
    <style>
        body { font-family: sans-serif; padding: 40px; }
    </style>
</head>
<body>
    <h1>Hello from PHP!</h1>
    <p>Rust server says: <strong><?= htmlspecialchars($data['message']) ?></strong></p>
</body>
</html>
