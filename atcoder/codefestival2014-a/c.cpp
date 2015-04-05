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

ll check(ll y) {
	ll s = y / 400;
	s += y/4;
	s -= y/100;
	return s;
}

int main(void){
	int a,b;
	cin>>a>>b;
	cout << check(b) - check(a-1) << endl;
}
