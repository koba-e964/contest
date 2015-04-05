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

typedef long long ll;
using namespace std;
const int mod=1000000007;
const int N=2000;
int a[N];

int inv(int x) {
	ll sum = 1;
	ll cur =x;
	int c=mod-2;
	while(c) {
		if(c%2){
			sum*=cur;
			sum%=mod;
		}
		cur*=cur;
		cur%=mod;
		c/=2;
	}
	return sum;
}

int hcalc(int a,int b) {
	ll s = 1;
	REP(i,0,b){
		s*=a+i;
		s%=mod;
		s*=inv(i+1);
		s%=mod;
	}
	return (int)s;
}

int main(){
	int n;
	cin >>n;
	int s=0,c=0;
	ll pp = 1;
	REP(i,0,n){
		cin>>a[i];
		if(a[i]==-1){
			if(c==0){
				s=a[i-1];
				c=1;
			} else {
				c++;
			}
		} else {
			if (c>=1){
				pp *= hcalc(a[i]-s+1,c);
				pp %= mod;
				c=0;
			}
		}
	}
	cout << pp << endl;
	return 0;
}
