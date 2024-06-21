package main

import (
	"context"
	"fmt"
	"html/template"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
)

var tmpl *template.Template

func init() {
	// Parse the template from a separate file
	var err error
	tmpl, err = template.ParseFiles("tpl/index.html")
	if err != nil {
		log.Fatal("error parsing template", err)
	}
}

type indexTplData struct {
	Inst         string
	ForwardedFor string
}

func indexHandler(w http.ResponseWriter, r *http.Request) {
	err := tmpl.Execute(w, indexTplData{
		Inst:         r.Header.Get("X-Tuc-Inst"),
		ForwardedFor: r.Header.Get("X-Tuc-Fwd-For"),
	})
	if err != nil {
		log.Println("failed to execute template", err)
	}
}

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("/", indexHandler)

	port := os.Getenv("PORT")
	if port == "" {
		log.Fatal("must provide PORT environment variable")
	}
	srv := &http.Server{
		Addr:    fmt.Sprintf(":%s", port),
		Handler: mux,
	}

	// Graceful shutdown
	done := make(chan os.Signal)
	signal.Notify(done, syscall.SIGINT, syscall.SIGTERM)

	go func() {
		<-done
		log.Println("shutting down server...")
		if err := srv.Shutdown(context.Background()); err != nil {
			log.Fatal("error during shutdown", err)
		}
	}()

	log.Printf("(%d) server listening at port %s", os.Getpid(), port)
	srv.ListenAndServe()
}
