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

#define DEBUG 0
const int N=100000;
int len;
int buf[N];


int calc(){
	bool all=true;
	REP(i,0,len){//1?
		all&=buf[i]==1;
	}
	if(all)return -1;
	all=true;
	REP(i,0,len){//0?
		all&=buf[i]==0;
	}
	if(all)return -1;
	int m=1;
	int state=buf[0];
	int cur=1;
	REP(i,1,2*len){
		if(buf[i%len]==state){
			cur++;
			m=max(m,cur);
		}
		else {
			state=buf[i%len];
			cur=1;
		}
	}
	return (m-1)/2+1;
}

int main(void){
	cin>>len;
	REP(i,0,len){
		cin>>buf[i];
	}
	cout << calc() <<endl;
}
