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
const double EPS=1e-9;

const ll MOD = 1e9+7;

ll x,y,n;
ll v[6];
int main(void){
	cin>>x>>y>>n;
	x%=MOD;
	x+=MOD;
	x%=MOD;
	y%=MOD;
	y+=MOD;
	y%=MOD;
	v[0] = x;
	v[1] = y;
	REP(i,2,6) {
		v[i] = v[i-1] - v[i-2] + MOD;
		v[i] %= MOD;
	}
	cout << v[(n + 5)%6] << endl;
}
