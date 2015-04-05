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

int n,m,a,b;

int main(void){
	cin>>n>>m >> a>>b;
	int r = n % m, q = n / m;
	int res = q * min(b, m*a) + min(r * a,b);
	cout << res << endl;
}
