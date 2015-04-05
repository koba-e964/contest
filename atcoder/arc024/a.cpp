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
const int N=100;

int l,r;
int ll[N];
int rr[N];
int vv[100];
int ww[100];
int main(void){
	cin >> l>> r;
	REP(i,0,l) cin>>ll[i];
	REP(i,0,r) cin>>rr[i];
	REP(i,0,100){
		vv[i]=0;
		ww[i]=0;
	}
	REP(i,0,l){
		vv[ll[i]]++;
	}
	REP(j,0,r){
		ww[rr[j]]++;
	}
	int sum=0;
	REP(i,0,100){
		sum += min(vv[i], ww[i]);
	}
	cout << sum << endl;
	return 0;
}
