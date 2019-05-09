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

const int N = 2014;
int n,k;
int f[N];

int main(void){
	cin>>n>>k;
	REP(i,0,n) {
		cin>>f[i];
	}
	sort(f,f+n);
	int p = n-1;
	ll sum = 0;
	while(p >= 0) {
		sum += 2 * (f[p] - 1);
		p -= k;
	}
	cout << sum;
}
