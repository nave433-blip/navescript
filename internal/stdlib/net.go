package stdlib

import (
    "io/ioutil"
    "net/http"
    "time"
)

var NetModule = map[string]interface{}{
    "get": httpGet,
}

func httpGet(url string) (string, error) {
    client := &http.Client{Timeout: 10 * time.Second}
    resp, err := client.Get(url)
    if err != nil {
        return "", err
    }
    defer resp.Body.Close()
    body, err := ioutil.ReadAll(resp.Body)
    return string(body), err
}
