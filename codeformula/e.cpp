#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;

const int N = 24;
const int B = 1 << 23;

string s;

int fib[N];

void fib_init(void) {
	int x = 0;
	int y = 1;
	REP(i,0,N) {
		fib[i] = x;
		int r = x + y;
		x = y;
		y = r;
	}
}

int dp[N][B];

// if not found, return -1.
int rec(int k,int b) {
	int &r = dp[k][b];
	if(r >= -1) return r;
	if(k == 1) {
		return s.substr(b,fib[k]) == "b" ? 0 : -1;
	}
	if(k == 2) {
		return s.substr(b,fib[k]) == "a" ? 0 : -1;
	}
	// [k-3] [k-2] [k-2]
	int f3 = fib[k-3];
	int f2 = fib[k-2];
	if(s.substr(b + f3,f2) == s.substr(b + f3 + f2, f2)) {
		int sub = rec(k-2, b + f3);
		if(sub >= 0)
			return sub * 4 + 1;
	}
	if(s.substr(b ,f2) == s.substr(b + f3 + f2, f2)) {
		int sub = rec(k-2, b + f3);
		if(sub >= 0)
			return sub * 4;
	}
	if(s.substr(b,f2) == s.substr(b + f2, f2)) {
		int sub = rec(k-2, b + f3);
		if(sub >= 0)
			return sub * 4 + 2;
	}
	return -1;
}

int main(void){
	cin>>s;
	fib_init();
	int n;
	if(s == "a") {
		cout << "2 0" << endl;
		return 0;
	}
	if(s == "b") {
		cout << "1 0" << endl;
		return 0;
	}
	REP(i,0,N) {
		if(s.size() == fib[i]) {
			n=i;
			break;
		}
	}
	cout << n << " " << rec(0,n) << endl;
}
