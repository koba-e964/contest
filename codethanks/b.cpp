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

int n,a[3];

int main(void){
	cin>>n>>a[0]>>a[1]>>a[2];	
	int t = a[0]+a[1]+a[2];
	sort(a,a+3);
	reverse(a,a+3);
	int q = n /t;
	int r = n - q * t;
    	int w = 0;
	if (r > a[0] + a[1]) {
		w=3*q+3;
	} else if (r > a[0]) {
		w=3*q+2;
	} else if (r > 0) {
		w =3 * q + 1;
	} else w=3*q;
	cout << w << endl;
	
}
