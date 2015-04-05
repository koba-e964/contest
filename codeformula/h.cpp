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

const int N = 50010;
int n,s1,s2;
int a[N],b[N];
int main(void){
	cin >> n >> s1 >> s2;
	REP(i,0,n) {
		cin >> a[i] >> b[i];
	}
	if (n >= 5000) return 1;
	ll c=0;
	REP(i,0,n) {
		REP(j,i+1,n) {
			ll tt = ll(a[i] - a[j]) * (b[i] - b[j]);
			if (tt >= s1 && tt <= s2) {
				c ++;
			}
		}
	}
	cout <<c  << endl;
}
