This module contains code to generate the body that is sent to the /verify endpoint. The script first makes a request to /inputs?client=browser. This returns a dictionary in the following format: 
```
{
	"challenge": {
		"input": "eyJ2ZXJzaW9uIjoxLCJ1YmlkIjoiMDY0YzRkOGMtZGQ4My00MjZhLTgxMzUtMWE2YWI0MzhiZmFkIiwiYXR0ZW1wdF9pZCI6ImU4N2E2OGI1LTEwNWQtNGFhZC04ZmMwLWFiYjJhZTUwZDEwNSIsImNyZWF0ZV90aW1lIjoiMjAyNC0xMC0yOVQxMzowMzo0Mi42NTU0ODMwNDJaIiwiZGlmZmljdWx0eSI6NCwiY2hhbGxlbmdlX3R5cGUiOiJIYXNoY2FzaFNjcnlwdCJ9",
		"hmac": "hgprBLBITcozNm1S9lRfhvcoi0O5Y9P8YV5EF+cikz0=",
		"region": "us-east-1"
	},
	"challenge_type": "h72f957df656e80ba55f5d8ce2e8c7ccb59687dba3bfb273d54b08a261b2f3002",
	"difficulty": 4
}
```
The POW can be one of two hash types, scrypt or sha256. This is determined by the challenge_type field. 
The difficulty represents the complexity of the generated hash. Most times it is either 4 or 8.

After making this request, the challenge script does the following
1. Calculates the number of forms and form elements on the page
2. Collects client-side browser signals(canvas, webgl, etc..)
3. Computes the pow result 

This data is concatenated into a json body and then encrypted via AES-GCM. 

The iv bytes, tag bytes, and cipher text bytes are appended together in this format:

BASE64encodedIV::HexEncodedTag::HexEncodedCipherText, 

representing the KramerAndRio Key. 

Metrics : 

This dictionary contains calculated numbers representing unique info about each of the run collectors(canvas, webgl, etc). Not completely sure what they do so Ive randomized them for now(not the greatest solution) 



