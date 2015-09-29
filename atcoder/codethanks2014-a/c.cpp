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

const int N=100;
int n,m;
int a[N];
int b[N];
int main(void){
	cin>>n>>m;
	REP(i,0,n) cin>>a[i];
	REP(i,0,m) cin >> b[i];
	int t = 0;
	REP(i,0,m) {
		t += a[b[i]-1];
	}
	cout << t << endl;
}
