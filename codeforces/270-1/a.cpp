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

int n;

void t(int x) {
	cout << x << " "  << n-x << endl;
}

int main(void){
	cin>>n;
	int tt[6] = {6,4,8,9,10,9};
	t(tt[n%6]);
	return 0;
}
