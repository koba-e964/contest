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
const double EPS=1e-9;

const int N=1000000;

char bar[N];
int main(void){
	int cnt=0;
	while(1){
		int ch=getchar();
		if(ch==EOF)break;
		if(ch<=' ')continue;
		bar[cnt]=ch;
		cnt++;
	}
	ll wsum=0;
	ll sum=0;
	int piv=0;
	REP(i,0,cnt){
		if(bar[i]>='0' && bar[i]<='9'){
			wsum+=(ll)(bar[i]-'0')*(ll)i;
			sum+=(ll)(bar[i]-'0');
		}
		if(bar[i]=='^'){
			piv=i;
		}
	}
	ll res=wsum-sum*piv;
	if(res<0)cout<<"left"<<endl;
	if(res==0)cout<<"balance"<<endl;
	if(res>0)cout<<"right"<<endl;
}
