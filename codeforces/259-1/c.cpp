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

const int M=100001;
const int N=100001;
int n,m;

int u[M],v[M];
int x[N];
int main(void){
	cin>>n>>m;
	REP(i,0,m) {
		cin>>u[i] >> v[i];
	}
	REP(i,0,n) {
		cin >> x[i];
	}
}
