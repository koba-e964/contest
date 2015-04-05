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


int xx1,yy1,xx2,yy2;
int main(void){
	cin>>xx1>>yy1>>xx2>>yy2;
	if(xx1 == xx2) {
		int ddd = yy1 - yy2;
		cout << xx1 + ddd << " " << yy1 << " " << xx2 + ddd << " " << yy2 << endl;
		return 0;
	}
	if(yy1 == yy2) {
		int ddd = xx1 - xx2;
		cout << xx1 << " " << yy1 +ddd << " " << xx2 << " " << yy2 +ddd << endl;
		return 0;
	}
	int dxx = xx1 - xx2;
	int dyy = yy1 - yy2;
	if (dxx*dxx == dyy*dyy) {
		cout << xx1 << " " << yy2 << " " << xx2 << " " << yy1 << endl;
	} else cout << -1 << endl;
}
