#!/bin/bash
set -e

score_sum=0
time_sum=0
count=0

# 並列実行して一時ファイルに結果を保存
for i in $(seq 0 49)
do
    num=$(printf "%04d" $i)
    cargo run --release --bin ahc043-a < in/${num}.txt > tmp_${num}.txt &
    sleep 0.15
done

# すべてのプロセスの完了を待つ
wait

# 結果の集計
for i in $(seq 0 49)
do
    num=$(printf "%04d" $i)
    time=$(cat tmp_${num}.txt | grep "#time:" | awk '{print $2}')
    score=$(cat tmp_${num}.txt | grep "#score:" | awk '{print $2}')
    
    time_sum=$((time_sum + time))
    score_sum=$((score_sum + score))
    count=$((count + 1))
    
    rm tmp_${num}.txt
done

if [ $count -gt 0 ]; then
    echo "Total Score: $score_sum"
    echo "Average Time: $((time_sum / count))"
fi

