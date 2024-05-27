FROM denoland/deno:debian

EXPOSE 8000

WORKDIR /app

USER deno

ADD . .
RUN deno cache main.ts

CMD ["run", "-A", "main.ts"]
