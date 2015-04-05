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


string s;
int main(void){
	cin >> s;
	ll cnt = 0;
	if (s.length() % 2) {
		cout << "No" << endl;
		return 0;
	}
	REP(i,0,s.length()) {
		if (s[i] == '-') {
			cnt += i % 2 ? 1 : -1;
		}
	}
	cout << (cnt == 0 ? "Yes" : "No") << endl;
}
