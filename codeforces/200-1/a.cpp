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

ll rec(ll a, ll b) {
	if (a == 0 && b == 1) {
		return 0;
	}
	if (a < b) {
		return rec(b,a);
	}
	return a / b + rec(a % b, b);
}


int main(void){
	ll a,b;
	cin>>a>>b;
	cout << rec(a,b) << endl;
}
