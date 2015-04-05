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

const int N=1000001;

char buf[N];
int c1,c6,c8,c9=0;
int main(void){
	int rem=0;
	int cnt=0;
	while(1){
		int ch=getchar();
		if(ch==EOF)break;
		if(ch<=0x20)continue;
		if(ch==1&&!c1){c1++;continue;}
		if(ch==6&&!c6){c1++;continue;}
		if(ch==8&&!c8){c1++;continue;}
		if(ch==9&&!c9){c1++;continue;}
		buf[cnt]=ch;
		cnt++;
	}
	int bias[]={1,3,2,6,4,5};//10^n
	REP(i,0,cnt){
		rem+=(buf[i]-'0')*bias[(cnt-1-i)%6];
		rem%=7;
	}
	int tbl[]={1869,1968,1689,6198,1698,1986,1896,};
	int ansrem=(7-rem)*bias[(6-(cnt%6))%6];
	ansrem%=7;
	cout<<tbl[ansrem];
	REP(i,0,cnt)cout<<buf[i];
	cout<<endl;
}
