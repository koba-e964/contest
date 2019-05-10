#include <stdio.h>
#include <stdlib.h>
#include <map>
#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;

void fail(){
  cout<<"No"<<endl;
  exit(0);
}

void reg(map<char,char>&a,char b,char c){
  if(a.count(b)){
    if(a[b]!=c)fail();
  }else a[b]=c;
}

int main(void){
  string s,t;
  cin>>s>>t;
  map<char,char>tbl,inv;
  int n=s.size();
  REP(i,0,n){
    reg(tbl,s[i],t[i]);
    reg(inv,t[i],s[i]);
  }
  cout<<"Yes"<<endl;
}
