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

int a, b;
int n;
int main(void){
	int t;
	a = b = 0;
	REP(i,0,3) {
		cin >> t;
		a += t;
	}
	REP(i,0,3) {
		cin >> t;
		b += t;
	}
	cin >> n;
	if ((a+4)/5 + (b+9)/10 <= n) {
		cout << "YES" <<endl;
	} else {
		cout << "NO" <<endl;
	}
}
