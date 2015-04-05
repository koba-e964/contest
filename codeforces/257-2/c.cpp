#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

vector<int> divisor(int val) {
	vector<int> v;
	v.reserve(800);
	for(int i=1; i*i <= val; i++) {
		if(val % i == 0) {
			v.push_back(i);
			if( i*i != val) {
				v.push_back(val/i);
			}
		}
	}
	sort(v.begin(), v.end());
	return v;
}


ll n,m,k;

ll check(int p) {
	int q = k - p;
	if(p<0 || q <0) {
		return -1;
	}
	ll sum = n / (p+1);
	sum *= m / (q+1);
	return sum;
}
int main(void){
	cin>>n>>m>>k;
	if (k >= n+m-1) {
		cout << -1 << endl;
		return 0;
	}
	ll m = 0;
	for(int i=1;(i-2)*(i-2)<=n; i++) {
		m = max(m, check(i-1));
		m = max(m, check(n/i-1));
	}
	cout << m << endl;
}
