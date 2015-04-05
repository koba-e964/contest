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


int n,m;
int v[101];
int main(void){
	cin>>n>>m;
	REP(i,0,n) {
		cin>>v[i];
	}
	int maxi = 0, maxv =0;
	REP(i,0,n) {
		int t= (v[i]+ m - 1)/m;
		if(maxv<=t) {
			maxi = i;
			maxv=t;
		}
	}
	cout<<(maxi + 1)<< endl;
}
