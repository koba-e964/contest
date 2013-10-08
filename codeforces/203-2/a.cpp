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

const int N=100;
int n,m;
int a[N],b[N];
int main(void){
	cin>>n>>m;
	REP(i,0,n)cin>>a[i];
	REP(i,0,m)cin>>b[i];
	int max0=0,min1=100000;
	REP(i,0,n){
		max0=max(max0,a[i]);
		min1=min(min1,a[i]*2);
	}
	max0=max(max0,min1);
	int min2=100000;
	REP(i,0,m)
		min2=min(min2,b[i]);
	if(max0>=min2){
		cout<<-1<<endl;
		return 0;
	}
	cout<<max0<<endl;
	return 0;
}
