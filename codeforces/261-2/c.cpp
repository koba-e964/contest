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


int n,k,d;
int main(void){
	cin>>n>>k>>d;
	if(n - 0.5 > pow(k,d)) {
		cout << "-1\n";
		return 0;
	}
	ll p = 1;
	REP(i,0,d) {
		REP(j,0,n) {
			if( p == 0) {
				cout << 1;
			} else {
				ll r = (j/p) %k;
				cout << (r+1);
			}
			cout << (j == n-1 ? "\n": " ");
		}
		p *=k;
		if(p > n) p = 0;
	}
}
