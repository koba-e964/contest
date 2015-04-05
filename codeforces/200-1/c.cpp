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

const int N = 100010;

int n,m;
ll h[N], p[N];

int main(void){
	cin>>n>>m;
	REP(i,0,n) {
		cin>>h[i];
	}
	REP(i,0,m) {
		cin >> p[i];
	}
}
