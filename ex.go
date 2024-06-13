package main

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"sync"
	"time"
)

// fetchURL makes a GET request to the provided URL and sends the response's status code to the channel.
func fetchURL(url string, ch chan<- string) {
	// Record the start time of the request.
	start := time.Now()

	// Make the GET request.
	resp, err := http.Get(url)
	if err != nil {
		ch <- fmt.Sprintf("Error fetching %s: %v", url, err)
		return
	}
	defer resp.Body.Close() // Ensure the response body is closed after processing.

	// Read the response body.
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		ch <- fmt.Sprintf("Error reading response from %s: %v", url, err)
		return
	}

	// Calculate the time taken for the request.
	secs := time.Since(start).Seconds()

	// Send a formatted string containing the URL, status code, bytes fetched, and time taken to the channel.
	ch <- fmt.Sprintf("%.2f seconds: %s %s %d [%d bytes]", secs, http.MethodGet, url, resp.StatusCode, len(body))
}

func main() {
	// A list of URLs to fetch.
	urls := []string{
		"http://google.com",
		"http://facebook.com",
		"http://youtube.com",
		"http://amazon.com",
		"http://twitter.com",
	}

	// Create a channel to communicate between the goroutines and the main function.
	ch := make(chan string)

	// Create a WaitGroup to wait for all goroutines to finish.
	var wg sync.WaitGroup

	// Start one goroutine for each URL.
	for _, url := range urls {
		wg.Add(1)
		go func(u string) {
			defer wg.Done()
			fetchURL(u, ch)
		}(url)
	}

	// Start a goroutine that closes the channel once all fetches are done.
	go func() {
		wg.Wait()
		close(ch)
	}()

	// Read from the channel as messages arrive.
	for message := range ch {
		fmt.Println(message)
	}

	fmt.Println("Fetched all URLs.")
}
