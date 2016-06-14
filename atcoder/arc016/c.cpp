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
const bool TEST = 0;

const int N=10;
const int M=4;


int c[M],cost[M];
double prob[M][N];
double dp[1 << N];
int idol[M][N];
double p[M][N];

int n,m;

double rec(int bits) {
  double& ret=dp[bits];
  if(ret>=0)return ret;
  if(bits==(1<<n)-1){
    return ret=0.0;
  }
  double mi = 1.0 / 0.0;
  REP(j, 0, m) {
    double vc = 1.0;
    double sum = 0;
    REP(i, 0, n){
      if(bits & (1<<i)){
	vc -= prob[j][i];
      }else{
	sum += prob[j][i] * rec(bits | (1 << i));
      }
    }
    if (vc <= 0) {
      continue;
    }
    mi = min(mi, (sum + cost[j]) / vc);
  }
  return ret = mi;
}

int main(void){
  cin>>n>>m;
  fill_n(dp, 1 << N, -1.0);
  REP(i,0,m){
    cin>>c[i]>>cost[i];
    REP(j,0,c[i]){
      cin>>idol[i][j]>>p[i][j];
      prob[i][idol[i][j]-1]=p[i][j]/100.0;
    }
  }
  if(TEST){
    REP(i,0,m){
      REP(j,0,n){
	cout<<prob[i][j]<<" ";
      }
      cout<<":"<<cost[i]<<endl;
    }
  }
  if(n==1){
    int mm=0x3ffffff;
    REP(i,0,m){
      if(mm>cost[i]){
	mm=cost[i];
      }
    }
    cout<<mm<<endl;
    return 0;
  }
  double sum = rec(0);
  if(TEST){
    REP(i,0,1 << n){
      cout<<"dp[" << hex << i << dec << "]=" << dp[i] << endl;
    }
  }
  printf("%.10f\n", sum);
  return 0;
	
}
