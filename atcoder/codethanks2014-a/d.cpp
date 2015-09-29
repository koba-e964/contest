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

int n,q;


int solve(int a, int b, int s, int t) {
	if (b <= s || t <= a) { // no intersection
		return t-s;
	}
	if(s <= a && b <= t) {
		return t- b + a -s;
	}
	if (b <= t) {
		return t- b;
	}
	if (s <= a) {
		return a-s;
	}
	return 0;
}

int main(void){
	cin >> n >> q;
	REP(i,0,q) {
		int a, b, s, t;
		cin >> a >> b>> s>> t;
		cout << solve(a,b,s,t)  * 100 << endl;
	}
}
