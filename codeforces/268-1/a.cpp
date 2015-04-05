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

void ooo(int x,int y,char op, int z) {
	cout << x << " " << op << " " << y << " = " << z << endl;
}

int main(void){
	int n;
	cin>>n;
	if(n <= 3) {
		cout << "NO" << endl;
		return 0;
	}
	cout << "YES" << endl;
	if(n % 2 == 0) {
		for(int i = n; i >= 6; i -= 2) {
			ooo(i,i-1, '-', 1);
			ooo(1,i-2, '*', i-2);
		}
		ooo(4,3,'*',12);
		ooo(12,2,'*',24);
		ooo(24,1,'*',24);
		return 0;
	}
	for(int i = n; i >= 7; i -= 2) {
		ooo(i,i-1, '-', 1);
		ooo(1,i-2, '*', i-2);
	}
	ooo(5,1,'-',4);
	ooo(4,2,'-',2);
	ooo(2,3,'*',6);
	ooo(6,4,'*',24);
}
