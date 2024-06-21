package main

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"log"
	"math/big"
	"net/http"
	"strings"
	"time"
)

func trialDivision(n *big.Int) []*big.Int {
	var factors []*big.Int
	zero := big.NewInt(0)
	one := big.NewInt(1)
	two := big.NewInt(2)

	for new(big.Int).Mod(n, two).Cmp(zero) == 0 {
		factors = append(factors, new(big.Int).Set(two))
		n.Div(n, two)
	}

	i := big.NewInt(3)
	for new(big.Int).Mul(i, i).Cmp(n) <= 0 {
		for new(big.Int).Mod(n, i).Cmp(zero) == 0 {
			factors = append(factors, new(big.Int).Set(i))
			n.Div(n, i)
		}
		i.Add(i, two)
	}

	if n.Cmp(one) > 0 {
		factors = append(factors, new(big.Int).Set(n))
	}

	return factors
}

func hashFactors(factors []*big.Int) string {
	var sb strings.Builder
	for _, factor := range factors {
		sb.WriteString(factor.String())
	}
	hash := sha256.Sum256([]byte(sb.String()))
	return hex.EncodeToString(hash[:])
}

type Response struct {
	Fatores []string `json:"fatores"`
	Sha     string   `json:"sha"`
}

func factorHandler(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("number")
	if param == "" {
		http.Error(w, "Missing 'number' parameter", http.StatusBadRequest)
		return
	}

	num := new(big.Int)
	_, ok := num.SetString(param, 10)
	if !ok {
		http.Error(w, "Invalid 'number' parameter", http.StatusBadRequest)
		return
	}

	factorsChan := make(chan []*big.Int)
	go func() {
		factorsChan <- trialDivision(num)
	}()

	select {
	case factors := <-factorsChan:
		var factorsStr []string
		for _, factor := range factors {
			factorsStr = append(factorsStr, factor.String())
		}

		hash := hashFactors(factors)

		response := Response{
			Fatores: factorsStr,
			Sha:     hash,
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(response)

	case <-time.After(200 * time.Second):
		http.Error(w, "Request timed out", http.StatusRequestTimeout)
	}
}

func main() {
	http.HandleFunc("/factors", factorHandler)

	fmt.Println("Server is listening on port 8080...")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
