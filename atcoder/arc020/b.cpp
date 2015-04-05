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

int n,c;
int a[100];

int check(int x,int y){
	int d=0;
	int ary[2]={x,y};
	REP(i,0,n){
		if(ary[i%2] != a[i]) d++;
	}
	return d;
}

int main(void){
	cin>>n>>c;
	REP(i,0,n) cin>>a[i];
	int m=0x3fffffff;
	REP(i,1,11){
		REP(j,1,11){
			if(i==j)continue;
			m=min(m,check(i,j));
		}
	}
	cout<<(m*c)<<endl;
	return 0;
}
