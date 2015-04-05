#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;


const int N=100010;
int n,m;

int boss[N];
int be[N];

int rank[N];

int pack[N+10]; // packid -> mem
int pc = 0;

int que[N][2];
int qc = 0;

int fold[N];
int f2[N];

void crank(int i) {
	if (boss[i] == -1) {
		rank[i] = 0;
		return;
	}
	crank(boss[i]);
	rank[i] = rank[boss[i]] + 1;
}

void up(int v, int d) {
	int cur=v;
	REP(i,0,d) {
		fold[v] = max(fold[v],be[cur]);
	}
	f2[v] = cur;
}

int check(int x,int p) {
	int cur = pack[p];
	while(rank[cur] != 0) {
		if (fold[cur] >p ) return 0;
		cur = f2[cur];
	}
	return 1;
}

int main(void){
	cin>>n>>m;
	REP(i,0,n+1) {
		boss[i] = -1;
	}
	REP(i,0,m) {
		int t,x,y;
		scanf("%d",&t);
		switch(t){
		case 1:
			scanf("%d%d",&x,&y);
			boss[x] = y;
			be[x] = pc;
			break;
		case 2:
			scanf("%d",&x);
			pc++;
			pack[pc]= x;
			break;
		case 3: //query
			scanf("%d%d",&x,&y);
			que[qc][0] = x;
			que[qc][1] = y;
			qc++;
			break;
		}
	}
	REP(i,1,n+1) {
		rank[i] = -1;
	}
	REP(i,1,n+1) {
		crank(i);
	}
	REP(i,1,n+1) {
		int r = rank[i];
		up(i, r & (-r));
	}
	REP(i,0,qc) {
		printf("%s\n", check(que[i][0], que[i][1]) ? "YES" : "NO");
	}
}
